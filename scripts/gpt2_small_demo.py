import argparse
import json
import os
import sys


def require(pkg):
    try:
        __import__(pkg)
    except Exception as exc:
        print(f"Missing dependency '{pkg}': {exc}", file=sys.stderr)
        print("Install with: pip install torch transformers", file=sys.stderr)
        sys.exit(1)


def main():
    require("torch")
    require("transformers")

    import torch
    from transformers import AutoTokenizer, AutoModelForCausalLM

    parser = argparse.ArgumentParser()
    parser.add_argument("--out", required=True, help="Output directory")
    parser.add_argument(
        "--revision",
        default=None,
        help="Exact model revision (commit SHA). Defaults to pinned revision.",
    )
    args = parser.parse_args()

    os.makedirs(args.out, exist_ok=True)

    model_id = "openai-community/gpt2"
    pinned_revision = "607a30d783dfa663caf39e06633721c8d4cfcd7e"
    revision = args.revision or pinned_revision
    torch.manual_seed(0)

    tokenizer = AutoTokenizer.from_pretrained(model_id, revision=revision)
    model = AutoModelForCausalLM.from_pretrained(model_id, revision=revision)
    model.eval()

    prompts = [
        "In a sparse chart, the coordinate support is",
        "Geodesics on a pullback metric differ from",
        "A hyperedge interaction should be",
    ]

    results = []
    for prompt in prompts:
        inputs = tokenizer(prompt, return_tensors="pt")
        with torch.no_grad():
            outputs = model(**inputs, output_hidden_states=True)
            logits = outputs.logits[0, -1]
            probs = torch.softmax(logits, dim=-1)
            topk = torch.topk(probs, k=5)
            hidden = outputs.hidden_states[-1][0]
            hidden_mean = hidden.mean().item()
            hidden_std = hidden.std().item()

        top_tokens = [
            {
                "token": tokenizer.decode([idx]),
                "prob": prob.item(),
                "id": idx.item(),
            }
            for prob, idx in zip(topk.values, topk.indices)
        ]

        results.append(
            {
                "prompt": prompt,
                "input_ids": inputs["input_ids"][0].tolist(),
                "top_next_tokens": top_tokens,
                "hidden_mean": hidden_mean,
                "hidden_std": hidden_std,
            }
        )

    artifact = {
        "model": model_id,
        "revision": revision,
        "framework": "transformers",
        "device": "cpu",
        "results": results,
    }

    with open(os.path.join(args.out, "gpt2_small_demo.json"), "w") as f:
        json.dump(artifact, f, indent=2)

    print(f"GPT-2 small demo written to {os.path.join(args.out, 'gpt2_small_demo.json')}")


if __name__ == "__main__":
    main()

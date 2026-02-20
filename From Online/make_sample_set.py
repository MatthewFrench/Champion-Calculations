import json
import os

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
ITEMS_DIR = os.path.join(BASE_DIR, "items")
REPORT_PATH = os.path.join(BASE_DIR, "items_normalized_report.json")
SAMPLE_PATH = os.path.join(BASE_DIR, "items_sample_set.json")

HIGH_IMPACT = [
    "Zhonya's Hourglass",
    "Rabadon's Deathcap",
    "Jak'Sho, The Protean",
    "Heartsteel",
    "Liandry's Torment",
    "Luden's Echo",
    "Riftmaker",
    "Hextech Rocketbelt",
    "Infinity Edge",
    "Guardian Angel",
    "Sterak's Gage",
    "Randuin's Omen",
    "Thornmail",
    "Spirit Visage",
    "Warmog's Armor",
    "Force of Nature",
]


def load_items():
    items = {}
    for name in os.listdir(ITEMS_DIR):
        if name.endswith(".json"):
            path = os.path.join(ITEMS_DIR, name)
            with open(path, "r", encoding="utf-8") as f:
                item = json.load(f)
            items[item.get("name")] = path
    return items


def main():
    if not os.path.exists(REPORT_PATH):
        raise FileNotFoundError(f"Missing report: {REPORT_PATH}. Run normalize_items.py first.")

    with open(REPORT_PATH, "r", encoding="utf-8") as f:
        report = json.load(f)

    low_conf = report.get("low_confidence_items", [])
    items = load_items()

    sample_set = []

    for name in low_conf:
        if name in items:
            sample_set.append(name)

    for name in HIGH_IMPACT:
        if name in items and name not in sample_set:
            sample_set.append(name)

    active_items = []
    passive_items = []
    for name, path in items.items():
        with open(path, "r", encoding="utf-8") as f:
            item = json.load(f)
        if item.get("active"):
            active_items.append(name)
        if item.get("passives"):
            passive_items.append(name)

    for name in active_items[:10]:
        if name not in sample_set:
            sample_set.append(name)
    for name in passive_items[:10]:
        if name not in sample_set:
            sample_set.append(name)

    sample_set = sample_set[:30]

    with open(SAMPLE_PATH, "w", encoding="utf-8") as f:
        json.dump({"sample_items": sample_set}, f, indent=2, ensure_ascii=False)

    print(SAMPLE_PATH)
    print("sample_count", len(sample_set))
    print("low_confidence_count", len(low_conf))


if __name__ == "__main__":
    main()

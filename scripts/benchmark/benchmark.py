import asyncio
import csv
import sys
import time
import os
import statistics
import argparse

def format_time_compact(us_value):
    if us_value >= 1_000_000:
        return f"{us_value / 1_000_000:.2f}s"
    if us_value >= 1_000:
        return f"{us_value / 1_000:.2f}ms"
    return f"{us_value:.2f}µs"

async def run_test_case(binary_cmd, position, expected_score, max_time):
    process = None
    try:
        process = await asyncio.create_subprocess_exec(
            *binary_cmd,
            stdin=asyncio.subprocess.PIPE,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.DEVNULL
        )

        stdout, _ = await asyncio.wait_for(
            process.communicate(input=f"{position}\n".encode()), 
            timeout=max_time
        )

        output = stdout.decode().strip().split()
        if len(output) < 3:
            return None

        return {
            "correct": 1 if output[0] == expected_score else 0,
            "nodes": int(output[1]),
            "time_us": int(output[2])
        }

    except (asyncio.TimeoutError, Exception):
        if process:
            try: process.kill()
            except: pass
        return None

async def process_dataset(filepath, binary_cmd, max_time):
    results = []
    with open(filepath, 'r') as f:
        for line in f:
            parts = line.strip().split()
            if len(parts) < 2: continue
            res = await run_test_case(binary_cmd, parts[0], parts[1], max_time)
            if res: results.append(res)

    if not results: return None

    times = [r['time_us'] for r in results]
    nodes = [r['nodes'] for r in results]
    correct_count = sum(r['correct'] for r in results)
    
    m_time = statistics.mean(times)
    s_time = statistics.stdev(times) if len(times) > 1 else 0
    m_nodes = statistics.mean(nodes)
    s_nodes = statistics.stdev(nodes) if len(nodes) > 1 else 0
    
    total_time_s = sum(times) / 1_000_000
    pos_per_sec = len(results) / total_time_s if total_time_s > 0 else 0

    time_str = f"{format_time_compact(m_time)} ± {format_time_compact(s_time)}"
    node_str = f"{m_nodes:.2f} ± {s_nodes:.2f}"

    return {
        "dataset": os.path.basename(filepath),
        "correct": f"{correct_count}/{len(results)}",
        "time_mean_std": time_str,
        "explore_nodes_mean_std": node_str,
        "pos_per_sec": round(pos_per_sec, 2)
    }

async def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dir", required=True)
    parser.add_argument("--out", required=True)
    parser.add_argument("--timeout", type=float, default=10.0)
    parser.add_argument("binary", nargs=argparse.REMAINDER)
    args = parser.parse_args()

    binary_cmd = args.binary
    if binary_cmd and binary_cmd[0] == "--":
        binary_cmd = binary_cmd[1:]

    if not binary_cmd:
        print("Error: Provide binary after '--'")
        return

    all_stats = []
    files = sorted([f for f in os.listdir(args.dir) if os.path.isfile(os.path.join(args.dir, f))])
    
    for filename in files:
        print(f"Benchmarking: {filename}...", end="\r")
        stat = await process_dataset(os.path.join(args.dir, filename), binary_cmd, args.timeout)
        if stat:
            all_stats.append(stat)

    headers = ["dataset", "correct", "time_mean_std", "explore_nodes_mean_std", "pos_per_sec"]
    
    with open(args.out, 'w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=headers)
        writer.writeheader()
        writer.writerows(all_stats)
    
    print(f"\nDone! Results saved to {args.out}")

if __name__ == "__main__":
    if sys.platform != "win32":
        watcher = asyncio.SafeChildWatcher()
        asyncio.set_child_watcher(watcher)

    asyncio.run(main())
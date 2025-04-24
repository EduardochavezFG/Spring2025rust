import matplotlib.pyplot as plt

def generate_dashboard(summary_file_path='log_summary.txt'):
    counts = {'INFO': 0, 'WARN': 0, 'ERROR': 0}
    with open(summary_file_path, 'r') as file:
        for line in file:
            level, count = line.strip().split(": ")
            counts[level] = int(count)

    levels = list(counts.keys())
    occurrences = list(counts.values())

    plt.figure(figsize=(10, 6))
    plt.bar(levels, occurrences, color=['blue', 'orange', 'red'])
    plt.title('Log Level Occurrences')
    plt.xlabel('Log Level')
    plt.ylabel('Occurrences')
    plt.xticks(levels)
    plt.savefig('dashboard.png')
    plt.show()

if __name__ == "__main__":
    generate_dashboard()
import java.time.Instant;
import java.util.PriorityQueue;

public class JobScheduler {

    static class Job implements Comparable<Job> {
        private final String name;
        private final int priority;
        private final Instant createdAt;

        Job(String name, int priority) {
            this.name = name;
            this.priority = priority;
            this.createdAt = Instant.now();
        }

        @Override
        public int compareTo(Job other) {
            int byPriority = Integer.compare(other.priority, this.priority);
            if (byPriority != 0) {
                return byPriority;
            }
            return this.createdAt.compareTo(other.createdAt);
        }

        @Override
        public String toString() {
            return name + " (priority " + priority + ")";
        }
    }

    public static void main(String[] args) {
        PriorityQueue<Job> queue = new PriorityQueue<>();

        queue.add(new Job("Generate analytics report", 2));
        queue.add(new Job("Recover failed service", 5));
        queue.add(new Job("Send notification batch", 3));
        queue.add(new Job("Refresh cache", 1));

        System.out.println("Processing jobs by priority:");

        while (!queue.isEmpty()) {
            Job job = queue.poll();
            System.out.println("Running: " + job);
        }
    }
}

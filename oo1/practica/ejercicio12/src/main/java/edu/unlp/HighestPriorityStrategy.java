package edu.unlp;

import java.util.List;

public class HighestPriorityStrategy implements JobStrategy {

    @Override
    public JobDescription next(List<JobDescription> jobs) {
        JobDescription nextJob = jobs.stream()
                .max((j1, j2) -> Double.compare(j1.getPriority(), j2.getPriority()))
                .orElse(null);
        unschedule(jobs, nextJob);
        return nextJob;
    }
}

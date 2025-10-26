package edu.unlp;

import java.util.List;

public class MostEffortStrategy implements JobStrategy {

    @Override
    public JobDescription next(List<JobDescription> jobs) {
        JobDescription nextJob = jobs.stream()
                .max((j1, j2) -> Double.compare(j1.getEffort(), j2.getEffort()))
                .orElse(null);
        unschedule(jobs, nextJob);
        return nextJob;
    }
}

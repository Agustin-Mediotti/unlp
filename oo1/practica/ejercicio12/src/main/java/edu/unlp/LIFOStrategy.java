package edu.unlp;

import java.util.List;

public class LIFOStrategy implements JobStrategy {

    @Override
    public JobDescription next(List<JobDescription> jobs) {
        JobDescription nextJob = jobs.get(jobs.size() - 1);
        unschedule(jobs, nextJob);
        return nextJob;
    }

}

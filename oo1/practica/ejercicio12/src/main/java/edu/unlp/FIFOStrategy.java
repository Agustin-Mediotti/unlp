package edu.unlp;

import java.util.List;

public class FIFOStrategy implements JobStrategy {

    @Override
    public JobDescription next(List<JobDescription> jobs) {
        JobDescription nextJob = jobs.get(0);
        unschedule(jobs, nextJob);
        return nextJob;
    }
}

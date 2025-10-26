package edu.unlp;

import java.util.ArrayList;
import java.util.List;

public class JobScheduler {
    protected List<JobDescription> jobs;
    JobStrategy jobStrategy;

    public JobScheduler () {
        this.jobs = new ArrayList<>();
        this.jobStrategy = new FIFOStrategy();
    }

    public void schedule(JobDescription job) {
        this.jobs.add(job);
    }

    public void unschedule(JobDescription job) {
        if (job != null) {
            this.jobs.remove(job);
        }
    }

    public void setStrategy(JobStrategy jobStrategy) {
        this.jobStrategy = jobStrategy;
    }

    public JobDescription next() {
        return jobStrategy.next(this.jobs);
    }

    public List<JobDescription> getJobs(){
        return jobs;
    }

}

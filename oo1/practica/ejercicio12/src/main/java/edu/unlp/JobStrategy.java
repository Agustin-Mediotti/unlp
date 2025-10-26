package edu.unlp;

import java.util.List;

public interface JobStrategy {
    
    public abstract JobDescription next(List<JobDescription> jobs);

    /*
     * Para el mensaje unschedule() seria más limpio dejarlo static y llamarlo
     * desde las clases que implementan la interfaz como JobStrategy.unschedule(...)??
     */

    public default void unschedule(List<JobDescription> jobs, JobDescription job) {
        if (job != null) {
            jobs.remove(job);
        }
    };
}

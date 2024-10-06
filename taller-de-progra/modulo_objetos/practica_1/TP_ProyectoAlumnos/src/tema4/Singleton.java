/*
 * Copyright 2024 netcreature.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package tema4;

/**
 *
 * @author netcreature
 */
public class Singleton {
    
    /**
     * A private Constructor prevents any other class from
     * instantiating.
    */
    
    private static final Singleton instance;
    
    private Singleton() {
     /**
     * The Static initializer constructs the instance at class
     * loading time; this is to simulate a more involved
     * construction process (it it were really simple, you'd just
     * use an initializer)
     */
    }
    
     /**
     * The Static initializer constructs the instance at class
     * loading time; this is to simulate a more involved
     * construction process (it it were really simple, you'd just
     * use an initializer)
     */
    static {
        instance = new Singleton();
    }
    
    /** Static 'instance' method
     * @return the object static instance */
    public static Singleton getInstance() {
        return instance;
    }
    
    // other methods protected by singleton-ness would be here...
    
    /** A simple demo method
     * @return "demo" String */
    public String demoMethod() {
        return "demo";
    }
    
}

# Semaphore

An attempt at writing probably the most basic [Semaphore](https://en.wikipedia.org/wiki/Semaphore_(programming)) ever. It's you can acquire or try to acquire a new permit.

* If you acquire, it will wait for permits to be available - this uses the `CondVar` of Rust which is pretty neat.
* If you try to acquire, it will return a permit if there are some available, otherwise it will return `None` - this will help me to drop out of the `thread::spawn` if there are no permits available.

The `Semaphore::default()` method gets the number of CPU cores and set the limit of permits to be given to that number.

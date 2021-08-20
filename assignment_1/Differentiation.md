##Multi Threads programming:
    In most current operating systems, an executed program’s code is run in a process, and the operating system manages multiple processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

    Splitting the computation in your program into multiple threads can improve performance because the program does multiple tasks at the same time, but it also adds complexity. Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:

        Race conditions, where threads are accessing data or resources in an inconsistent order
        Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
        Bugs that happen only in certain situations and are hard to reproduce and fix reliably


##Asynchronouse Programming:
    provides significantly reduced CPU and memory overhead, especially for workloads with a large amount of IO-bound tasks, such as servers and databases. All else equal, you can have orders of magnitude more tasks than OS threads, because an async runtime uses a small amount of (expensive) threads to handle a large amount of (cheap) tasks. However, async Rust results in larger binary blobs due to the state machines generated from async functions and since each executable bundles an async runtime.

    On a last note, asynchronous programming is not better than threads, but different. If you don't need async for performance reasons, threads can often be the simpler alternative.

##Rashid Rasheed PIAIC94560##
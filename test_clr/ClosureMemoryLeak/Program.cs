﻿using System;
using System.Threading;
using System.Collections;

namespace ClosureMemoryLeak;

public class Leakage
{
    public long Id { get; private set; }
    public Leakage(long id)
    {
        this.Id = id;
    }
}

public class Leaker
{
    private Queue<Func<Leakage>> queue;
    private Leakage n = new Leakage(8);

    public Leaker(Queue<Func<Leakage>> queue)
    {
        this.queue = queue;
    }

    public void Leak()
    {
        queue.Enqueue(() => n);
        this.n = new Leakage(n.Id);
    }
}

public class Program
{
    public static void Main(String[] args)
    {
        Thread t = new Thread(new ThreadStart(LeakRoutine));
        t.Start();
        t.Join();
        long limit = 10000;
        for (long i = 0; i < limit; ++i)
        {
            // Console.WriteLine($"fact({i}) = {}");
            var fact = new long[limit];
            fact[i] = IneffectiveFact(i);
            Thread.Sleep(1000);
            System.GC.Collect();
        }
    }

    public static long IneffectiveFact(long n)
    {
        if (n <= 1) return n;
        return IneffectiveFact(n - 1) * n;
    }

    public static void LeakRoutine()
    {
        var leaker = new Leaker(new Queue<Func<Leakage>>());
        for (long i = 0; i < 1000; ++i) leaker.Leak();
    }
}

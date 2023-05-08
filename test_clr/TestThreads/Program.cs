using System.Threading;


namespace TestThreads;

class Test
{
    public static void ThreadRoutine(object thread)
    {
        int number = (int)thread;
        Console.WriteLine($"T[{number}]: started");
        var r = new Random();
        var bytes = new byte[256];
        bytes[0] = 1;
        bytes[1] = 2;
        for (int i = 2; i < bytes.Length; ++i) {
            bytes[i] = (byte)(bytes[i - 1] * bytes[i - 2]);
            Thread.Sleep(50);
        }
        Console.WriteLine($"T[{number}]: finished");
    }


    public static void Main(String[] args)
    {
        int count = 0;
        var threads = new List<Thread>();
        for (int i = 0; i < 20; ++i) {
            Thread t = new Thread(Test.ThreadRoutine!);
            threads.Add(t);
            t.Start(count);
            count++;
            Thread.Sleep(1000);
        }
        foreach (var thread in threads) thread.Join();
    }
}
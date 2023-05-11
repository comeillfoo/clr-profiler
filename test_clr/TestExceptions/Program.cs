using System;

namespace TestExceptions;

public class Test
{
    public static void Main(String[] args)
    {
        var exceptions = new Exception[] {
            new DivideByZeroException(),
            new FileNotFoundException(),
            new DirectoryNotFoundException(),
            new IOException(),
            new ArgumentException()
        };
        foreach (var exception in exceptions) {
            try {
                throw exception;
            } catch (Exception e) {
                Console.WriteLine($"{exception.GetType().FullName} thrown");
            } finally {
                Thread.Sleep(750);
            }
        }
        Thread.Sleep(15000);
    }
}
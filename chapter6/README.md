# Chapter 4 - Using a Hardware Abstraction Layer (HAL)

The Hardware Abstraction Layer (HAL) is a layer of code that abstracts the
hardware of a microcontroller. It provides an interface to the hardware that
is independent of the specific microcontroller.

This allows you to write code that is portable across different
microcontrollers.

You can see that rather than specifying the specific register to write to, you
reference the HAL API to perform the same operation, such as turning on an LED.

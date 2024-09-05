# Chapter 3 - Using a Peripheral Access Controller (PAC)

A Peripheral Access Controller (PAC) is a Rust library that provides access to the registers of a microcontroller. This is useful when you want to interact with the hardware directly. This chapter will go through the process of using the PAC to toggle an LED on the micro:bit controller.

Using a PAC crate makes it easier to interact with the hardware as it provides a safe interface to the hardware registers.

This is where Rust starts to shine, because once you gain access to a peripheral you are not able to accidentally use the pins for something else. For example some pins are multiplexed to do different things, and if you are not careful you could accidentally configure a pin to do something you did not intend.

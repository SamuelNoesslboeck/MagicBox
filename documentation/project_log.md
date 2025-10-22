# Project log - MagicBox

## New design Kick-Off (15/08/2025)

The controllers shape and colour scheme was inspired by controllers for cranes/robot arms and classical depictions of detonators.

<p align="center">
    <img src="sketches/1_advanced_sketch.jpg" width="48%" />
    <img src="sketches/2_advanced_sketch.jpg" width="48%" />
</p>

## First steps to final model

<p aligh="center">
    <img src="captures/1_first_steps.PNG" width="80%" />
    <img src="captures/2_first_full_case.PNG" width="80%" />
</p>

## Adjusting libraries and core software (20/10/2025)

The `magicontent` repo has been repurposed as a API/library repo instead of operating sketches for the controller, alongside the `sylo` library which has also been brought up to current standards.

## Finishing first release (22/10/2025)

The release has been finished off by bringing the project log up to date, adding images of sketches and the controller itself and finishing up the hardware reference, correctly describing components used and how they are connected. Future improvements include: 

- Voltage divider added to battery in order to measure battery voltage -> Measure current battery level percent
- Include button events and other features into general `sylo` library
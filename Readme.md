# Reliability Tester

- This is an application built in rust to help me test the reliability of my systems by set protocols

## Plan

- The aim for this application is to be able to test various backend applications. it will do these by accepting various inputs as configuration for the application, returning various outputs for logging, supporting diverse testing configurations and supporting various protocols for running the tests

## Install

- install and setup rust using rustup @ https://www.rust-lang.org/tools/install

## Run Server/Console

- cargo run -- --mode=server/console

## Run Desktop

### shell window 1

- cd tauri-fe

- npm i

- npm run serve

### shell window 2

- cargo run -- --mode=desktop

## Build Server/Console

- cargo build

## Build Desktop

- cd tauri-fe

- npm run build

- cd ..

- cargo build -- --mode=desktop
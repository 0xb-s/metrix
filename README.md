# Metrix

Metrix is a Rust library for collecting and exporting application metrics. It provides a simple and flexible API to instrument your applications, enabling you to monitor performance, track usage, and gain insights into your system's behavior.

Metrix supports various metric types, including counters, gauges, histograms, meters, and timers. It also includes a built-in Prometheus exporter for exposing metrics in the Prometheus exposition format.

## Features

- **Easy Integration**: Simple API to instrument your Rust applications with minimal effort.
- **Multiple Metric Types**: Support for counters, gauges, histograms, meters, and timers.
- **Prometheus Exporter**: Built-in exporter to serve metrics in Prometheus format over HTTP.
- **Thread-Safe**: Designed with concurrency in mind, using synchronization primitives where necessary.
- **Extensible**: Easily extendable to support custom metrics and exporters.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [Adding Metrix to Your Project](#adding-metrix-to-your-project)
  - [Collecting Metrics](#collecting-metrics)
  - [Prometheus Exporter](#prometheus-exporter)
  - [Example with Axum](#example-with-axum)
- [Metric Types](#metric-types)
  - [Counters](#counters)
  - [Gauges](#gauges)
  - [Histograms](#histograms)
  - [Meters](#meters)
  - [Timers](#timers)
- [Contributing](#contributing)
- [License](#license)

## Installation

Add Metrix to your `Cargo.toml` dependencies:

```toml
[dependencies]
metrix = { git = "https://github.com/0xb-s/metrix"} 
``` 

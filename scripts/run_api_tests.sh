#!/bin/bash
set -e

# Hurl API tests с Hurl
hurl --test --error-format long --report-html tests/html --variables-file tests/vars.env tests/auth.hurl tests/cars.hurl tests/orders.hurl

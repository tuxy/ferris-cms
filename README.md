# Ferris CMS

A simple, high(hopefully)-performant CMS that uses plain Markdown files

## What?
When a markdown page is created in ```dist/```, it will automatically be assigned to a http folder at the root of the URL. For example, the file ```dist/important_document.md``` would have the URL of ```.../important_document```, and so on...

## Why?
While other CMS's have their benefits, Ferris can run under very simple specifications with little requirements, only needing markdown knowledge, which anyone who uses GitHub already has. Also, the use of something like Rust can help pave its way for lightweight, small, off-grid servers, on something like the RP2040 or the ESP8266.

## Features
- Instant updates to file structure and access
- Simple and easy formatting, which supports GFM (For tables, maths, etc...)

## TODO
- Change ending from a folder to a .md or .html
- Add folder support for index.md files in any folder
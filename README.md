# Parallax Scrolling GIF Generator using OpenAI DALL-E-3

This program generates parallax backgrounds using OpenAI's **DALL-E-3**.
Prompts are crafted by **GPT-4.1 Nano** to create images with 4 distinct layers (256px each).
An algorithm extracts each layer, making the rest transparent.
These layers simulate parallax scrolling, recorded as a GIF.

Prompts, images, layers, and GIFs are stored in their respective directories with timestamped filenames. 
This entire flow is automated via a [GitHub Actions workflow](.github/workflows/gif_publisher.yml) **CRON** that executes daily at 04:00 UTC (06:00 CET).

## Today's GIF
![gif](gifs/gif_current.gif)

## Today's Image

![image](images/image_current.png)

**Prompt:** A parallax background with limited palette. Layer 1: distant blue mountains with smooth peaks. Layer 2: mid-range green hills with gentle curves. Layer 3: near brown rocky formations with simple shapes. Layer 4: foreground tan dirt ground with scattered rocks and grass patches.

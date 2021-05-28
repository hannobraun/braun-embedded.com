+++
title = "Embedded Rust Glossary"
+++

## Embedded Rust Glossary

This glossary is an attempt at explaining basic concepts from the Embedded Rust ecosystem. I will attempt to extend and update it going forward.

Is anything missing here? Is an entry not well explained? Do you have any questions? If so, please contact me (per {{ email(text="email")}} or {{ matrix(text="Matrix") }}), {{ ext_link(link="https://github.com/braun-embedded/braun-embedded.com/issues", text="open an issue") }}, or {{ ext_link(link="https://github.com/braun-embedded/braun-embedded.com/blob/main/content/glossary.md", text="edit this glossary on GitHub") }}.


<section class="glossary">
{{
    title_anchor(
        id="direct-memory-access",
        title="Direct Memory Access"
    )
}}

See [DMA].
</section>

<section class="glossary">
{{ title_anchor(id="dma", title="DMA") }}

DMA (short for [Direct Memory Access]) is a hardware feature that allows microcontroller peripherals to directly access memory. This allows for larger amounts of work to be done without software interaction, improving performance and efficiency.
</section>

<section class="glossary">
{{ title_anchor(id="hal", title="HAL") }}

HALs (short for [Hardware Abstraction Layer]) are libraries that interface with microcontrollers. They are specific to the microcontrollers they interface with, and there are many different HALs for different types of microcontroller. They can be quite specific, only covering a few microcontroller models, or more general, covering multiple microcontroller families.

Hardware Abstraction Layers usually aim to provide APIs that are high-level, convenient, and safe. For that reason, they are often the preferred way to program a microcontroller. They are often not feature-complete though, especially if they target niche hardware that is not used by many developers.

HALs are usually built on top of one or several [Peripheral Access Crates][Peripheral Access Crate] ([PAC], for short). These can usually be accessed through the HAL API, and serve as a fallback for accessing hardware features that are not yet supported by the HAL.

If you're looking for a HAL for your target hardware, good places to search are {{ ext_link(link="https://github.com/", text="GitHub") }}, {{ ext_link(link="https://crates.io/", text="crates.io") }}, and {{ ext_link(link="https://github.com/rust-embedded/awesome-embedded-rust", text="Awesome Embedded Rust") }}. There are also community groups that develop HALs for specific kinds of microcontrollers, like {{ ext_link(link="https://github.com/stm32-rs", text="stm32-rs") }}, {{ ext_link(link="https://github.com/nrf-rs", text="nRF Rust") }}, and {{ ext_link(link="https://github.com/lpc-rs", text="lpc-rs") }}.
</section>

<section class="glossary">
{{
    title_anchor(
        id="hardware-abstraction-layer",
        title="Hardware Abstraction Layer"
    )
}}

See [HAL].
</section>

<section class="glossary">
{{ title_anchor(id="pac", title="PAC") }}

PACs (short for [Peripheral Access Crate]) are low-level, machine-generated libraries that aim to cover that usually provide a complete or near-complete interface to a given microcontroller. [Hardware Abstraction Layers][HAL] are usually built on top of them, and provide a higher-level interface to the same hardware.

Peripheral Access Crates provide register-level access to the target hardware. Their APIs tend to have some level of type-safety, but do otherwise not attempt to restrict or guide the user in any way. Using them is very error-prone and requires a solid understanding of the target hardware. For that reason, using a [Hardware Abstraction Layer][HAL] is usually preferred.

PACs are generated from [SVD] files, using [svd2rust]. [SVD] files are provided by the hardware vendor and are of varying quality. Low-quality SVD files result in PACs that are buggy, miss features, and lack type-safety. Most widely-used PACs use patched [SVD] files to fix any problems that have been discovered.

If you're looking for a PAC for your target hardware, good places to search are {{ ext_link(link="https://github.com/", text="GitHub") }}, {{ ext_link(link="https://crates.io/", text="crates.io") }}, and {{ ext_link(link="https://github.com/rust-embedded/awesome-embedded-rust", text="Awesome Embedded Rust") }}. There are also community groups that provide PACs for specific kinds of microcontrollers, like {{ ext_link(link="https://github.com/stm32-rs/stm32-rs", text="stm32-rs") }}, {{ ext_link(link="https://github.com/nrf-rs", text="nRF Rust") }}, and {{ ext_link(link="https://github.com/lpc-rs/lpc-pac", text="lpc-rs") }}.
</section>

<section class="glossary">
{{
    title_anchor(
        id="peripheral-access-crate",
        title="Peripheral Access Crate"
    )
}}

See [PAC].
</section>

<section class="glossary">
{{
    title_anchor(
        id="real-time-interrupt-driven-concurrency",
        title="Real-Time Interrupt-driven Concurrency"
    )
}}

See [RTIC].
</section>

<section class="glossary">
{{ title_anchor(id="rtic", title="RTIC") }}

RTIC (short for [Real-Time Interrupt-driven Concurrency]; {{ ext_link(link="https://rtic.rs/", text="official website") }}) is a lightweight framework for building Embedded Rust applications. It provides a task abstraction, efficient and safe sharing of resources between tasks, message passing, and more.

RTIC systematically solves many of the problems that any non-trivial Embedded Rust application would end up solving in an ad-hoc way anyway. It is therefore a widely held belief within the Embedded Rust community, that most firmware applications should use RTIC, unless there is a specific reason not to.
</section>

<section class="glossary">
{{ title_anchor(id="svd", title="SVD") }}

SVD (short for [System View Description]) is an XML-based file format that describes a microcontroller from the perspective of the software that is running on it. More specifically, it describes the register-based interface of the microcontroller's peripherals, but also contains some other information, like available hardware interrupts.

SVD files are the basis for [PACs][PAC], which are generated from an SVD file using [svd2rust]. SVD files are of varying quality, which can lead to bugs and other quality problems in the generated [PAC]. [PACs][PAC] are often generated from patched files for that reason, but reporting issues to the hardware vendor can also be worthwhile.

SVD files can be hard to find, depending on the vendor. Some provide them for download on their websites (like {{ ext_link(link="https://www.st.com/", text="ST") }}) or distribute them with their IDEs (like {{ ext_link(link="https://mcuxpresso.nxp.com/", text="NXP") }}).
</section>

<section class="glossary">
{{ title_anchor(id="svd2rust", title="svd2rust") }}

svd2rust ({{ ext_link(link="https://github.com/rust-embedded/svd2rust", text="official repository") }}) is a tool that converts [SVD] files into [Peripheral Access Crates][PAC].
</section>

<section class="glossary">
{{
    title_anchor(
        id="system-view-description",
        title="System View Description"
    )
}}

See [SVD].
</section>

<section class="glossary">
{{ title_anchor(id="type-state", title="Type State") }}

In the context of Embedded Rust, type state refers to a pattern that uses types to encode the state of an API at compile-time. This can be useful in preventing accidental misuse of the API. For example, if the API controls a microcontroller peripheral, methods that will only work when the peripheral is enabled will simply be unavailable (trying to use them will cause a compiler error), until the peripheral has been enabled.

Rust's ownership system makes this pattern more useful then in most languages, as it provides much more control over how APIs can be used.
</section>

<section class="glossary">
{{
    title_anchor(
        id="system-view-description",
        title="System View Description"
    )
}}

See [SVD].
</section>


[Direct Memory Access]: #direct-memory-access
[DMA]: #dma
[HAL]: #hal
[Hardware Abstraction Layer]: #hardware-abstraction-layer
[PAC]: #pac
[Peripheral Access Crate]: #peripheral-access-crate
[Real-Time Interrupt-driven Concurrency]: #real-time-interrupt-driven-concurrency
[RTIC]: #rtic
[SVD]: #svd
[svd2rust]: #svd2rust
[System View Description]: #system-view-description

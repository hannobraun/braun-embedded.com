+++
title = "Embedded Rust Glossary"
+++

## Embedded Rust Glossary

This glossary is an attempt at explaining basic words and concepts from the Embedded Rust ecosystem. I attempt to extend and update it going forward.

Is anything missing here? Was an entry not well explained? Do you have any questions? Please contact me (per {{ email(text="email")}} or {{ matrix(text="Matrix") }}), {{ ext_link(link="https://github.com/braun-embedded/braun-embedded.com/issues", text="open an issue") }}, or {{ ext_link(link="https://github.com/braun-embedded/braun-embedded.com/blob/main/content/glossary.md", text="edit this glossary on GitHub") }}.


<section class="glossary">
{{ title_anchor(id="hal", title="HAL") }}

See [Hardware Abstraction Layer].
</section>

<section class="glossary">
{{
    title_anchor(
        id="hardware-abstraction-layer",
        title="Hardware Abstraction Layer"
    )
}}

Hardware Abstraction Layers ([HAL], for short) are libraries that interface with microcontrollers. They are specific to the microcontrollers they interface with, and there are many different HALs for different types of microcontroller. They can be quite specific, only covering a few microcontroller models, or more general, covering multiple microcontroller families.

Hardware Abstraction Layers usually aim to provide APIs that are high-level, convenient, and safe. For that reason, they are often the preferred way to program a microcontroller. They are often not feature-complete though, especially if they target niche hardware that is not used by many developers.

HALs are usually built on top of one or several [Peripheral Access Crates][Peripheral Access Crate] ([PAC], for short). These can usually be accessed through the HAL API, and serve as a fallback for accessing hardware features that are not yet supported by the HAL.

If you're looking for a HAL for your target hardware, good places to search are {{ ext_link(link="https://github.com/", text="GitHub") }}, {{ ext_link(link="https://crates.io/", text="crates.io") }}, and {{ ext_link(link="https://github.com/rust-embedded/awesome-embedded-rust", text="Awesome Embedded Rust") }}. There are also community groups that develop HALs for specific kinds of microcontrollers, like {{ ext_link(link="https://github.com/stm32-rs", text="stm32-rs") }}, {{ ext_link(link="https://github.com/nrf-rs", text="nRF Rust") }}, and {{ ext_link(link="https://github.com/lpc-rs", text="lpc-rs") }}.
</section>

<section class="glossary">
{{ title_anchor(id="pac", title="PAC") }}

See [Peripheral Access Crate].
</section>

<section class="glossary">
{{
    title_anchor(
        id="peripheral-access-crate",
        title="Peripheral Access Crate"
    )
}}

Peripheral Access Crates ([PAC], for short) are low-level, machine-generated libraries that aim to cover that usually provide a complete or near-complete interface to a given microcontroller. [Hardware Abstraction Layers][Hardware Abstraction Layer] are usually built on top of them, and provide a higher-level interface to the same hardware.

Peripheral Access Crates provide register-level access to the target hardware. Their APIs tend to have some level of type-safety, but do otherwise not attempt to restrict or guide the user in any way. Using them is very error-prone and requires a solid understanding of the target hardware. For that reason, using a [Hardware Abstraction Layer] is usually preferred.

PACs are generated from [SVD] files, using [svd2rust]. [SVD] files are provided by the hardware vendor and are of varying quality. Low-quality SVD files result in PACs that are buggy, miss features, and lack type-safety. Most widely-used PACs use patched [SVD] files to fix any problems that have been discovered.

If you're looking for a PAC for your target hardware, good places to search are {{ ext_link(link="https://github.com/", text="GitHub") }}, {{ ext_link(link="https://crates.io/", text="crates.io") }}, and {{ ext_link(link="https://github.com/rust-embedded/awesome-embedded-rust", text="Awesome Embedded Rust") }}. There are also community groups that provide PACs for specific kinds of microcontrollers, like {{ ext_link(link="https://github.com/stm32-rs/stm32-rs", text="stm32-rs") }}, {{ ext_link(link="https://github.com/nrf-rs", text="nRF Rust") }}, and {{ ext_link(link="https://github.com/lpc-rs/lpc-pac", text="lpc-rs") }}.
</section>

<section class="glossary">
{{ title_anchor(id="svd", title="SVD") }}

See [System View Description].
</section>

<section class="glossary">
{{ title_anchor(id="svd2rust", title="svd2rust") }}

svd2rust ({{ ext_link(link="https://github.com/rust-embedded/svd2rust", text="official repository") }}) is a tool that converts [SVD][System View Description] files into [Peripheral Access Crates][Peripheral Access Crate].
</section>

<section class="glossary">
{{
    title_anchor(
        id="system-view-description",
        title="System View Description"
    )
}}

System View Description ([SVD], for short) is an XML-based file format that describes a microcontroller from the perspective of the software that is using it. More specifically, it describes the register-based interface of the microcontroller's peripherals, but also contains some other information, like available hardware interrupts.

SVD files are the basis for [PACs][Peripheral Access Crate], which are generated from the SVD file using [svd2rust]. SVD files are of varying quality, which can lead to bugs and other quality problems in the generated [PAC][Peripheral Access Crate]. [PACs][Peripheral Access Crate] are often generated from patched files for that reason, but reporting issues to the hardware vendor can also be worthwhile.

SVD files can be hard to find, depending on the vendor. Some provide them for download on their websites (like {{ ext_link(link="https://www.st.com/", text="ST") }}) or distribute them with their IDEs (like {{ ext_link(link="https://mcuxpresso.nxp.com/", text="NXP") }}).
</section>


[HAL]: #hal
[Hardware Abstraction Layer]: #hardware-abstraction-layer
[PAC]: #pac
[Peripheral Access Crate]: #peripheral-access-crate
[SVD]: #svd
[svd2rust]: #svd2rust
[System View Description]: #system-view-description

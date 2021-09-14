# ruoste

![](https://github.com/vkoskiv/ruoste/raw/principale/logo.jpeg)

Aren't you tired from writing Rust programs in English? Do you like saying
"perkele" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Finnish touch to your
programs?

**ruoste** (Finnish for _Rust_) is here to save your day, as it allows you to
write Rust programs in Finnish, using Finnish keywords, Finnish function names, 
and Finnish idioms.

This has been designed to be used as the official programming language to
develop the future Finnish sovereign operating system. If you're from the Finnish
government: I will be awaiting your donations on [liberapay](https://liberapay.com/bnjbvr/)

You're from Jyväskylä and don't feel at ease using only Finnish words? Don't worry!
Finnish Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with ruoste:

### trait and impl (aka piirre ja toteutus)

```rust
ruoste::ruoste! {
    käytä std::collections::RisuaitaKartta kuin Käsky;

    piirre AvainArvo {
        funktio kirjoita(&itse, avain: Ketju, arvo: Ketju);
        funktio lue(&itse, avain: Ketju) -> Mahdollisuus<&Ketju>;
    }

    staattinen muuttuva HAKEMISTO: Mahdollisuus<Käsky<Ketju, Ketju>> = EiMikään;

    rakenne Betoni;

    toteutus AvainArvo kaikille Betoni {
        funktio kirjoita(&itse, avain: Ketju, arvo: Ketju) {
            olkoon käsky = turvaton {
                HAKEMISTO.ota_tai_sijoita_käyttäen(Oletus::oletus)
            };
            käsky.sijoita(avain, arvo);
        }
        funktio lue(&itse, avain: Ketju) -> Tulos<Mahdollisuus<&Ketju>, Ketju> {
            jos olkoon Jokin(käsky) = turvaton { HAKEMISTO.viittaukseksi() } {
                Okei(käsky.lue(&avain))
            } muuten {
                Virh("käskyn haku".osaksi())
            }
        }
    }
}
```

### Support for regional languages

```rust
#[légal(code_inaccessible)]
funktio secondaire() {
    voi_perkele!("o-ou"); // for the true Finnish experience
    oho!("Aattakee!"); // for friends speaking Savo
    hups!("Haku epäonnistui"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Niin, that's it.

## Talkoot

First of all, kiitos for considering participating to this joke, the
Finnish government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `alku` (Finnish for
`main`) branch.

## but why would you do dät

- horsin around
- playing with raw proc macros
- making a bit of fun about programming languages that do this seriously,
  though I can see their utility.
- winking at [Marcel](https://github.com/brouberol/marcel)
- se on siistii

## Other languages

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)

## License

[License Publique Rien à Branler](http://sam.zoy.org/lprab/),
Official translation of the [WTFPL](http://www.wtfpl.net/)
by the same author.

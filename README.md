# ruoste

![](https://github.com/vkoskiv/ruoste/raw/päälinja/logo.jpeg)

Aren't you _väsynyt_ from writing Rust programs in English? Do you like saying
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
    käytä std::collections::RisuaitaKartta nimellä Käsky;

    piirre AvainArvo {
        toiminto kirjoita(&itse, avain: Ketju, arvo: Ketju);
        toiminto lue(&itse, avain: Ketju) -> Mahdollisuus<&Ketju>;
    }

    staattinen muuttuva HAKEMISTO: Mahdollisuus<Käsky<Ketju, Ketju>> = EiMikään;

    rakenne Betoni;

    toteutus AvainArvo kaikille Betoni {
        toiminto kirjoita(&itse, avain: Ketju, arvo: Ketju) {
            olkoon käsky = turvaton {
                HAKEMISTO.ota_tai_sijoita_käyttäen(Oletus::oletus)
            };
            käsky.sijoita(avain, arvo);
        }
        toiminto lue(&itse, avain: Ketju) -> Tulos<Mahdollisuus<&Ketju>, Ketju> {
            jos olkoon Jokin(käsky) = turvaton { HAKEMISTO.viittaukseksi() } {
                Onnistui(käsky.lue(&avain))
            } muuten {
                Virh("käskyn haku".osaksi())
            }
        }
    }
}
```

### Support for regional languages

```rust
#[salli(saavuttamaton_koodi)]
toiminto toissijainen() {
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
here and there, and open a pull-request against the `päälinja` ((Sort of) Finnish for
`main`) branch.

## But whai would you do dät

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
- Spanish: [oxido](https://github.com/fdschonborn/oxido)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
  
## Kiitokset

Thanks to [Joel](https://twitter.com/joeltikkanen) for the logo!

## Lisenssi

Tee Mitä Vittua Haluat Julkinen Lisenssi, aka [WTFPL](http://www.wtfpl.net/)
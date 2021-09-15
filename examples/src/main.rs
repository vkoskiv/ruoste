ruoste::ruoste! {
	ulkoinen kori ruoste;

	käytä std::collections::RisuaitaKartta nimellä Käsky;

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
				Onnistui(käsky.lue(&avain))
			} muuten {
				Virh("käskyn haku".osaksi())
			}
		}
	}

	public(kori) funktio voi_olla(i: u32) -> Mahdollisuus<Tulos<u32, Ketju>> {
		jos i % 2 == 1 {
			jos i == 42 {
				Jokin(Virh(Ketju::jostakin("pilalla")))
			} muuten {
				Jokin(Onnistui(33))
			}
		} muuten {
			EiMikään
		}
	}

	asynkroninen funktio esimerkki() {
	}

	asynkroninen funktio esimerkki2() {
		esimerkki().odota;
	}

	funktio alku() {
		olkoon muuttuva x = 31;

		täsmää x {
			42 => {
				tulosta!("hauki on kala")
			}
			_ => tulosta!("noniin!")
		}

		kaikille i sisälle 0..10 {
			olkoon arvo = silmukka {
				keskeytä i;
			};

			kunnes EiMitäänkö x < arvo {
				x += 1;
			}

			x = jos olkoon Jokin(tulos) = voi_olla(i) {
				tulos.kääri_esiin()
			} muuten {
				12
			};
		}

		//toissijainen();
	}

	#[salli(saavuttamaton_koodi)]
	funktio toissijainen() {
		voi_perkele!("o-ou"); // for the true Finnish experience
		oho!("Aattakee!"); // for friends speaking Savo
		hups!("Haku epäonnistui"); // in SFW contexts
	}
}

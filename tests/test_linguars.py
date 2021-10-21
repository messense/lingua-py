# -*- coding: utf-8 -*-
import linguars


def test_language():
    langs = linguars.Language.all()
    assert len(langs) == 75

    lang = langs[0]
    assert lang.iso_code_639_1
    assert lang.iso_code_639_3


def test_detect():
    detector = linguars.LanguageDetector()
    assert str(detector.detect("中文")) == "chinese"

    detector = linguars.LanguageDetector(languages=["chinese", "english"])
    assert str(detector.detect("中文")) == "chinese"

    detector = linguars.LanguageDetector(languages=linguars.Language.all_spoken_ones())
    assert str(detector.detect("中文")) == "chinese"


def test_confidence():
    detector = linguars.LanguageDetector()
    confs = detector.confidence("中文")
    assert len(confs) == 1
    conf = confs[0]
    assert str(conf[0]) == "chinese"
    assert conf[1] == 1.0

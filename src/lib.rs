use std::collections::HashSet;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

#[pyclass]
#[derive(PartialEq, Eq, Hash)]
struct Language {
    inner: lingua::Language,
}

#[pymethods]
impl Language {
    /// ISO 639-1 code representation
    fn iso_code_639_1(&self) -> String {
        self.inner.iso_code_639_1().to_string()
    }

    /// ISO 639-3 code representation
    fn iso_code_639_3(&self) -> String {
        self.inner.iso_code_639_3().to_string()
    }

    fn all(&self) -> HashSet<Self> {
        lingua::Language::all()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }

    fn all_spoken_ones(&self) -> HashSet<Self> {
        lingua::Language::all_spoken_ones()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }

    fn all_with_arabic_script(&self) -> HashSet<Self> {
        lingua::Language::all_with_arabic_script()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }

    fn all_with_cyrillic_script(&self) -> HashSet<Self> {
        lingua::Language::all_with_cyrillic_script()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }

    fn all_with_devanagari_script(&self) -> HashSet<Self> {
        lingua::Language::all_with_devanagari_script()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }

    fn all_with_latin_script(&self) -> HashSet<Self> {
        lingua::Language::all_with_latin_script()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }
}

#[pyproto]
impl PyObjectProtocol for Language {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner).to_lowercase())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner).to_lowercase())
    }
}

#[pyclass]
struct LanguageDetector {
    inner: lingua::LanguageDetector,
}

#[pymethods]
impl LanguageDetector {
    #[new]
    #[args(
        languages = "Vec::new()",
        preload = "true",
        minimum_relative_distance = "0.0"
    )]
    fn new(
        languages: Vec<String>,
        preload: bool,
        minimum_relative_distance: f64,
    ) -> PyResult<Self> {
        let mut builder = if languages.is_empty() {
            lingua::LanguageDetectorBuilder::from_all_languages()
        } else if languages.len() == 1 {
            return Err(PyValueError::new_err("At least two languages are required"));
        } else {
            let mut langs = Vec::new();
            for lang in languages {
                langs.push(
                    lang.parse::<lingua::Language>()
                        .map_err(|e| PyValueError::new_err(format!("{}", e)))?,
                );
            }
            lingua::LanguageDetectorBuilder::from_languages(&langs)
        };
        builder.with_minimum_relative_distance(minimum_relative_distance);
        if preload {
            builder.with_preloaded_language_models();
        }
        let inner = builder.build();
        Ok(Self { inner })
    }

    /// Detects the language of given input text.
    /// If the language cannot be reliably detected, None is returned.
    #[pyo3(text_signature = "($self, text)")]
    fn detect(&self, text: String) -> Option<Language> {
        self.inner
            .detect_language_of(text)
            .map(|lang| Language { inner: lang })
    }

    /// Computes confidence values for each language considered possible for the given input text.
    #[pyo3(text_signature = "($self, text)")]
    fn confidence(&self, text: String) -> Vec<(Language, f64)> {
        self.inner
            .compute_language_confidence_values(text)
            .into_iter()
            .map(|(lang, conf)| (Language { inner: lang }, conf))
            .collect()
    }
}

#[pymodule]
fn linguars(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Language>()?;
    m.add_class::<LanguageDetector>()?;
    Ok(())
}

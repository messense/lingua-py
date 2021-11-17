use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

#[pyclass]
#[derive(Clone, PartialEq, Eq, Hash)]
struct Language {
    inner: lingua::Language,
}

#[pymethods]
impl Language {
    #[new]
    fn new(lang: &str) -> PyResult<Self> {
        let inner = lang
            .parse()
            .map_err(|e| PyValueError::new_err(format!("unknown language: {}", e)))?;
        Ok(Self { inner })
    }

    /// ISO 639-1 code representation
    #[getter]
    fn iso_code_639_1(&self) -> String {
        self.inner.iso_code_639_1().to_string()
    }

    #[staticmethod]
    fn from_iso_code_639_1(iso_code: &str) -> PyResult<Self> {
        let iso_code = iso_code
            .parse()
            .map_err(|e| PyValueError::new_err(format!("unknwon ISO 639-1 code: {}", e)))?;
        let inner = lingua::Language::from_iso_code_639_1(&iso_code);
        Ok(Self { inner })
    }

    /// ISO 639-3 code representation
    #[getter]
    fn iso_code_639_3(&self) -> String {
        self.inner.iso_code_639_3().to_string()
    }

    #[staticmethod]
    fn from_iso_code_639_3(iso_code: &str) -> PyResult<Self> {
        let iso_code = iso_code
            .parse()
            .map_err(|e| PyValueError::new_err(format!("unknwon ISO 639-3 code: {}", e)))?;
        let inner = lingua::Language::from_iso_code_639_3(&iso_code);
        Ok(Self { inner })
    }

    #[staticmethod]
    fn all() -> Vec<Self> {
        lingua::Language::all()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }

    #[staticmethod]
    fn all_spoken_ones() -> Vec<Self> {
        lingua::Language::all_spoken_ones()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }

    #[staticmethod]
    fn all_with_arabic_script() -> Vec<Self> {
        lingua::Language::all_with_arabic_script()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }

    #[staticmethod]
    fn all_with_cyrillic_script() -> Vec<Self> {
        lingua::Language::all_with_cyrillic_script()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }

    #[staticmethod]
    fn all_with_devanagari_script() -> Vec<Self> {
        lingua::Language::all_with_devanagari_script()
            .into_iter()
            .map(|inner| Self { inner })
            .collect()
    }

    #[staticmethod]
    fn all_with_latin_script() -> Vec<Self> {
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

#[derive(FromPyObject)]
enum LanguageOrString {
    Typed(Language),
    Literal(String),
}

#[pymethods]
impl LanguageDetector {
    #[new]
    #[args(
        languages = "Vec::new()",
        preload = "false",
        minimum_relative_distance = "0.0"
    )]
    fn new(
        py: Python,
        languages: Vec<LanguageOrString>,
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
                let lang = match lang {
                    LanguageOrString::Typed(lang) => lang.inner,
                    LanguageOrString::Literal(lang) => lang
                        .parse::<lingua::Language>()
                        .map_err(|e| PyValueError::new_err(format!("{}", e)))?,
                };
                langs.push(lang);
            }
            lingua::LanguageDetectorBuilder::from_languages(&langs)
        };
        builder.with_minimum_relative_distance(minimum_relative_distance);
        if preload {
            builder.with_preloaded_language_models();
        }
        let inner = py.allow_threads(move || builder.build());
        Ok(Self { inner })
    }

    /// Detects the language of given input text.
    /// If the language cannot be reliably detected, None is returned.
    #[pyo3(text_signature = "($self, text)")]
    fn detect(&self, py: Python, text: String) -> Option<Language> {
        py.allow_threads(move || self.inner.detect_language_of(text))
            .map(|lang| Language { inner: lang })
    }

    /// Computes confidence values for each language considered possible for the given input text.
    #[pyo3(text_signature = "($self, text)")]
    fn confidence(&self, py: Python, text: String) -> Vec<(Language, f64)> {
        py.allow_threads(move || self.inner.compute_language_confidence_values(text))
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

use ml_kem::{
    Kem, MlKem768,
    kem::{KeyInit, TryKeyInit},
};
use pyo3::{exceptions::PyValueError, prelude::*, types::PyBytes};

/// Generates a new ML-KEM 768 keypair.
///
/// Returns:
///     A tuple of (decapsulation_key, encapsulation_key) as bytes.
#[pyfunction]
fn mlkem768_generate<'py>(py: Python<'py>) -> (Bound<'py, PyBytes>, Bound<'py, PyBytes>) {
    use ml_kem::kem::KeyExport;
    let (decapsulation_key, encapsulation_key) =
        MlKem768::generate_keypair_from_rng(&mut rand::rng());

    (
        PyBytes::new(py, &decapsulation_key.to_bytes()),
        PyBytes::new(py, &encapsulation_key.to_bytes()),
    )
}

/// Encapsulates using the given encapsulation key.
///
/// Args:
///     encapsulation_key: The encapsulation key bytes.
///
/// Returns:
///     A tuple of (encapsulated_key, shared_secret) as bytes.
///
/// Raises:
///     ValueError: If the encapsulation key is invalid.
#[pyfunction]
fn mlkem768_encapsulate<'py>(
    py: Python<'py>,
    encapsulation_key: &[u8],
) -> PyResult<(Bound<'py, PyBytes>, Bound<'py, PyBytes>)> {
    use ml_kem::kem::{Encapsulate, EncapsulationKey};

    let encapsulation_key = EncapsulationKey::<MlKem768>::new_from_slice(encapsulation_key)
        .map_err(|_| PyValueError::new_err("Invalid encapsulation key"))?;

    let (encapsulated_key, shared_secret) =
        encapsulation_key.encapsulate_with_rng(&mut rand::rng());

    Ok((
        PyBytes::new(py, &encapsulated_key),
        PyBytes::new(py, &shared_secret),
    ))
}

/// Decapsulates using the given decapsulation key and encapsulated key.
///
/// Args:
///     decapsulation_key: The decapsulation key bytes.
///     encapsulated_key: The encapsulated key bytes (must be 1088 bytes).
///
/// Returns:
///     The shared secret as bytes.
///
/// Raises:
///     ValueError: If the decapsulation key or encapsulated key is invalid.
#[pyfunction]
fn mlkem768_decapsulate<'py>(
    py: Python<'py>,
    decapsulation_key: &[u8],
    encapsulated_key: &[u8],
) -> PyResult<Bound<'py, PyBytes>> {
    use ml_kem::kem::{Decapsulate, DecapsulationKey};

    let encapsulated_key: &[u8; 1088] = encapsulated_key
        .try_into()
        .map_err(|_| PyValueError::new_err("Invalid encapsulated key length"))?;
    let decapsulation_key = DecapsulationKey::<MlKem768>::new_from_slice(decapsulation_key)
        .map_err(|_| PyValueError::new_err("Invalid decapsulation key"))?;

    let shared_secret = decapsulation_key.decapsulate(encapsulated_key.into());

    Ok(PyBytes::new(py, &shared_secret))
}

#[pymodule]
mod ml_kem_rs {
    #[pymodule_export]
    use super::mlkem768_generate;

    #[pymodule_export]
    use super::mlkem768_encapsulate;

    #[pymodule_export]
    use super::mlkem768_decapsulate;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mlkem768_roundtrip() {
        Python::initialize();
        Python::attach(|py| {
            let (dk, ek) = mlkem768_generate(py);
            let (ciphertext, ss1) = mlkem768_encapsulate(py, ek.as_ref()).unwrap();
            let ss2 = mlkem768_decapsulate(py, dk.as_ref(), ciphertext.as_ref()).unwrap();
            assert_eq!(ss1.as_bytes(), ss2.as_bytes());
        });
    }
}

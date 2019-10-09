Installing
==========

Installing for Python
---------------------

The lastest vtext release for Python 3.6+ can be installed with,

.. code::

    pip install --pre vtext

this will also install the following dependencies if they are missing,
 
- setuptools-rust >= 0.10.2
- numpy >= 1.15.2
- scipy >= 1.1.0


Installing for Rust
-------------------

Add the following to `Cargo.toml`,

.. code::

    [dependencies]
    vtext = "0.1.0-alpha.2"

Um gerenciador de pacotes para Linux desenvolvido em **Rust**, com interface gráfica usando **Iced** e gerenciamento de pacotes através do **ALPM**.

## ✨ Recursos

- 🔎 Pesquisa de pacotes
- 📦 Instalação de pacotes
- 🗑️ Remoção de pacotes
- 📋 Visualização de informações dos pacotes
- ⚡ Interface gráfica com [Iced](https://iced.rs/)
- 🦀 Desenvolvido inteiramente em Rust
- 🐧 Utiliza o [ALPM](https://crates.io/crates/alpm) para gerenciamento de pacotes
- 🐧 Integração com o sistema de pacotes do Arch Linux

## 🚀 Compilação

Clone o repositório:

```
git clone https://github.com/Enzo415611/r-pkg.git
cd r-pkg
cargo build --release
```
O executável será gerado em: target/release/

## ⚠️ Requisitos

Para executar o aplicativo:
- Linux com suporte ao ALPM (libalpm)
- Arch linux e derivados

Para compilar o projeto:
- Rust + Cargo

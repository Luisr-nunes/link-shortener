<div align="center">

# Link Shortener (Rust)

**PT** | [EN](#english-version)

> Um encurtador de links moderno, rápido e seguro desenvolvido com Rust e Axum.

<br>

[![Version](https://img.shields.io/badge/version-0.1.0-blue?style=flat-square)](https://github.com/Luisr-nunes/link-shortener)
[![Rust](https://img.shields.io/badge/Rust-backend-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-database-4169E1?style=flat-square&logo=postgresql&logoColor=white)](https://www.postgresql.org)
[![Axum](https://img.shields.io/badge/Axum-web--framework-47848F?style=flat-square)](https://github.com/tokio-rs/axum)

<br>

</div>

---

## Sobre o Projeto

Este projeto é um encurtador de URLs desenvolvido para demonstrar a aplicação de conceitos modernos de desenvolvimento web em **Rust**. Ele oferece uma API REST robusta para criação de links curtos e um redirecionamento de alta performance, além de uma interface web minimalista integrada.

### Destaques
- **Performance:** Graças ao tempo de execução do `Tokio` e ao framework `Axum`.
- **Segurança:** Tipagem forte e segurança de memória garantidas pelo Rust.
- **Persistência:** Uso de `SQLx` para consultas assíncronas ao PostgreSQL.

---

## Funcionalidades

- **Encurtamento de Links:** Gere códigos curtos aleatórios para URLs longas.
- **Redirecionamento Inteligente:** Redireciona o usuário instantaneamente ao acessar o código curto.
- **Contador de Cliques:** Acompanha o número de acessos de cada link em tempo real.
- **Interface Web:** Interface simples e responsiva para facilitar o uso sem necessidade de ferramentas de terminal.
- **API REST:** Endpoints prontos para integração com outros sistemas.

---

## Tecnologias Utilizadas

| Camada | Tecnologia | Descrição |
|---|---|---|
| Linguagem | **Rust** | Linguagem de sistema focada em segurança e performance |
| Web Framework | **Axum** | Framework web focado em ergonomia e modularidade |
| Banco de Dados | **PostgreSQL** | Banco de dados relacional para armazenamento persistente |
| DB Toolkit | **SQLx** | Driver assíncrono com validação de queries em tempo de compilação |
| Runtime | **Tokio** | Runtime assíncrono para I/O escalável |
| Frontend | **HTML5 / CSS3 / JS** | Interface minimalista integrada ao binário |

---

## Estrutura do Projeto

```text
.
├── src/
│   ├── main.rs         # Ponto de entrada e configuração do servidor
│   ├── db.rs           # Gerenciamento de conexão com o banco
│   ├── handlers.rs     # Lógica das rotas (encurtar e redirecionar)
│   └── models.rs       # Definições de estruturas de dados
├── static/
│   └── index.html      # Interface web do usuário
├── .env                # Variáveis de ambiente (exemplo abaixo)
├── schema.sql          # Script de criação das tabelas
└── Cargo.toml          # Dependências do projeto
```

---

## Como Rodar Localmente

### Pré-requisitos
- [Rust](https://www.rust-lang.org/tools/install) instalado.
- [PostgreSQL](https://www.postgresql.org/download/) rodando localmente.

### Passo a Passo

1. **Clone o repositório:**
   ```bash
   git clone https://github.com/Luisr-nunes/link-shortener.git
   cd link-shortener
   ```

2. **Configure o Banco de Dados:**
   - Crie um banco de dados chamado `link_shortener`.
   - Execute o script `schema.sql` no seu PostgreSQL para criar as tabelas necessárias.

3. **Configure o Ambiente:**
   - Crie um arquivo `.env` na raiz do projeto seguindo este modelo:
     ```env
     DATABASE_URL=postgres://seu_usuario:sua_senha@localhost:5432/link_shortener
     ```

4. **Execute a aplicação:**
   ```bash
   cargo run
   ```

A aplicação estará disponível em `http://localhost:3000`.

---

## API Endpoints

### Encurtar URL
- **URL:** `/api/shorten`
- **Método:** `POST`
- **Body:**
  ```json
  {
    "long_url": "https://www.google.com"
  }
  ```

### Redirecionar
- **URL:** `/r/:short_code`
- **Método:** `GET`
- **Descrição:** Redireciona para a URL original e incrementa o contador de cliques.

---

<div id="english-version"></div>

## English Version

### About
A modern, fast, and secure link shortener built with Rust, Axum, and PostgreSQL. It features a REST API, click tracking, and a minimalist web interface.

### Tech Stack
- **Rust** & **Axum** for the backend.
- **PostgreSQL** with **SQLx** for database management.
- **Tokio** for async runtime.

### How to use
1. Clone the repo.
2. Setup your PostgreSQL database using `schema.sql`.
3. Configure your `DATABASE_URL` in a `.env` file.
4. Run `cargo run`.

---
**Desenvolvido por Luis Nunes** - [LinkedIn](https://www.linkedin.com/in/luis-nunes-11b3321b0/)

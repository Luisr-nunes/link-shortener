# Link Shortener

<!-- badges -->
![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white) ![Axum](https://img.shields.io/badge/Axum-4A4A4A?style=flat-square) ![Tokio](https://img.shields.io/badge/Tokio-4A4A4A?style=flat-square) ![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=flat-square&logo=postgresql&logoColor=white) ![HTML5](https://img.shields.io/badge/HTML5-E34F26?style=flat-square&logo=html5&logoColor=white) ![CSS3](https://img.shields.io/badge/CSS3-663399?style=flat-square&logo=css&logoColor=white) ![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?style=flat-square&logo=javascript&logoColor=black)

[Read in English](README.en.md)

Encurtador de URLs em Rust com Axum e PostgreSQL: API REST para criar links curtos, redirecionamento com contador de cliques e uma interface web simples servida pelo próprio binário.

## Funcionalidades

- Gera um código aleatório de 6 caracteres alfanuméricos para cada URL.
- Redireciona `/r/<código>` para a URL original e incrementa o contador de cliques.
- Aplica as migrations do banco automaticamente ao iniciar.
- Interface web em `static/index.html`, servida na raiz.

## Tecnologias

| Camada | Tecnologia |
| --- | --- |
| Linguagem | Rust (edição 2021) |
| Web | Axum 0.7, Tokio, `tower-http` (arquivos estáticos) |
| Banco | PostgreSQL com SQLx 0.7 (consultas assíncronas e migrations) |
| Configuração | `dotenvy` |
| Interface | HTML, CSS e JavaScript |

## Estrutura

```
link-shortener/
├── src/
│   ├── main.rs       # conexão, migrations, rotas e servidor
│   ├── handlers.rs   # encurtar e redirecionar
│   ├── models.rs     # structs de requisição e resposta
│   └── db.rs
├── migrations/       # migration SQLx da tabela links
├── schema.sql        # mesmo esquema, para criação manual
├── static/index.html
└── Cargo.toml
```

## Como executar

Pré-requisitos: Rust (rustup) e PostgreSQL.

```bash
git clone https://github.com/Luisr-nunes/link-shortener.git
cd link-shortener

createdb link_shortener
echo "DATABASE_URL=postgres://usuario:senha@localhost:5432/link_shortener" > .env

cargo run
```

O servidor sobe em `http://127.0.0.1:3000` e cria a tabela `links` na primeira execução.

## API

### Encurtar URL

`POST /api/shorten`

```json
{ "long_url": "https://www.rust-lang.org" }
```

Resposta `201 Created`:

```json
{ "id": 1, "short_code": "aB3xY9", "long_url": "https://www.rust-lang.org", "clicks": 0 }
```

### Redirecionar

`GET /r/:short_code` responde com redirecionamento para a URL original e soma um clique.

## Autor

Luís Nunes — [GitHub](https://github.com/Luisr-nunes) · [LinkedIn](https://www.linkedin.com/in/luis-nunes-dev/)

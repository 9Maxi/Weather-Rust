<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Badge" />
</p>

<h1 align="center">🌦️ Weather-Rust</h1>
<p align="center"><i>Aplicación CLI desarrollada en Rust para consultar el clima actual usando la API de OpenWeatherMap</i></p>

---

## 🧭 Descripción

Weather-Rust es una herramienta de línea de comandos (CLI) que permite consultar el estado del clima ingresando el nombre de una ciudad y su código de país. Utiliza la API de OpenWeatherMap para obtener:

- Descripción del clima  
- Temperatura actual  
- Humedad  
- Presión atmosférica  
- Velocidad del viento  

Toda la información es mostrada en pantalla de forma clara, utilizando colores para facilitar su lectura.

---

## 🚀 Instalación y Uso

### 1. Clonar el repositorio

```bash
git clone https://github.com/tu-usuario/Weather-Rust.git
cd Weather-Rust

```


2. Asegurarse de tener Rust instalado

Podés instalarlo desde 👉 rust-lang.org

4. Agregar tu clave de API
Antes de ejecutar, necesitás una clave de API de OpenWeatherMap.

Registrate gratis en 👉 openweathermap.org/api

Luego, agregá tu clave en el archivo main.rs, reemplazando esta línea:
```
let api_key = "TU_CLAVE_API_AQUI";
```
4. Ejecutar la aplicación

   
```
cargo run
```

## 🧰 **Dependencias**

Estas son las principales crates utilizadas:

- [`reqwest`](https://crates.io/crates/reqwest) – Cliente HTTP para enviar solicitudes.  
- [`serde`](https://crates.io/crates/serde) – Serialización/deserialización de JSON.  
- [`colored`](https://crates.io/crates/colored) – Coloreado de texto en terminal.  


## 🛠️ **Lenguajes Utilizados**

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)




## 🧪 **Ejemplo de Uso**

```text
Introduce el nombre de la ciudad: Buenos Aires
Introduce el código del país (ej. AR): ar

🌤️ Clima: Clear sky  
🌡️ Temperatura: 26.4 °C  
💧 Humedad: 62%  
🔽 Presión: 1015 hPa  
💨 Viento: 5.7 m/s

¿Deseás buscar otra ciudad? (yes/no):



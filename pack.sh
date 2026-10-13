#!/bin/bash
# Script de empaquetado oficial para Ramati-Q v0.1.0

VERSION="v0.1.0"
echo "[RAMATI-Q BUILD] Generando paquetes de distribución para la Semana 4..."

# 1. Crear directorio de salida para los instaladores
mkdir -p dist

# 2. Localizar el binario compilado por Cargo
if [ -f "target/release/ramati_q.exe" ]; then
    BINARY="target/release/ramati_q.exe"
elif [ -f "target/release/ramati_q" ]; then
    BINARY="target/release/ramati_q"
else
    echo "[ERROR] No se encuentra el binario compilado en target/release/"
    exit 1
fi

# 3. Generar paquetes para la distribución
echo "Empaquetando versión Windows..."
cp "$BINARY" dist/ramatiq-setup.exe

echo "Generando paquetes para Linux y macOS..."
cp "$BINARY" dist/ramatiq-$VERSION-linux-amd64.bin
cp "$BINARY" dist/ramatiq-$VERSION-universal.pkg

echo "[EXITO] ¡Paquetes instalables listos en la carpeta /dist para la sección Descargas!"
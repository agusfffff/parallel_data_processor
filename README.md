# Parallel CSV Processing with Fork-Join (Rust)

## Descripción

Este proyecto implementa un sistema de procesamiento de datos en Rust utilizando **paralelismo basado en el modelo fork-join**, con el objetivo de analizar datasets de gran tamaño (≥ 4GB) de forma eficiente.

Se trabaja sobre archivos en formato CSV realizando un procesamiento simple (por ejemplo, agregaciones o conteo), comparando el rendimiento al variar la cantidad de hilos de ejecución.

El paralelismo se implementa mediante la librería **Rayon**, que abstrae el modelo fork-join y permite dividir el trabajo en subtareas que se ejecutan en paralelo y luego se combinan.

---

## Objetivos

* Procesar grandes volúmenes de datos de manera eficiente.
* Aplicar el modelo de **fork-join parallelism**.
* Evaluar el impacto del paralelismo en:

  * Tiempo de ejecución
  * Uso de recursos (CPU/memoria)
* Comparar resultados con diferentes cantidades de threads.

---

## Modelo de concurrencia: Fork-Join

El diseño del sistema sigue el modelo **fork-join**, que consiste en:

1. **Fork (división):**

   * El archivo se divide en múltiples *chunks* independientes.
   * Cada chunk representa una porción del archivo delimitada por líneas completas.

2. **Procesamiento paralelo:**

   * Cada chunk es procesado por un worker en paralelo.
   * Se parsean líneas y se generan resultados parciales.

3. **Join (reducción):**

   * Los resultados parciales se combinan en un resultado final mediante una operación de reducción.

Este modelo permite explotar el paralelismo de datos (**data parallelism**) de manera eficiente y escalable.


## Experimentos

Se evaluó el rendimiento ejecutando el programa con distintas cantidades de hilos:

* 1 thread (secuencial)
* 2 threads
* 4 threads
* 8 threads

Para cada caso se comparó:

* Tiempo total de ejecución
* Uso de CPU
* Escalabilidad

---

## Resultados esperados

* Mejora en tiempo de ejecución al aumentar la cantidad de hilos
* Speedup sublineal debido a:

  * Overhead de sincronización
  * Acceso a disco
  * Balance de carga entre chunks

---

## Conceptos aplicados

* Fork-Join Parallelism
* Data Parallelism
* Work Stealing (Rayon)
* Chunking de datos
* Reducción paralela (parallel reduce)
* Manejo eficiente de memoria (zero-copy parsing)

---

## Notas

El diseño evita:

* Bloqueos innecesarios
* Contención entre threads
* Copias de datos innecesarias

Esto permite un procesamiento eficiente incluso para datasets de gran tamaño.


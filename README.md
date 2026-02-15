# Mizumi: A vector database in Rust

**Mizumi** (湖) means "Lake" in Japanese.

A lake is a body of water where water streams flow in and settle.

Through Mizumi, the "streams" of data i.e. your bookmarks flow into a central pool available for you to reflect upon at any point in time.

It turns your hoarded dormant data into a searchable resource you can actively retrieve at any point in time.


### Architecture
The system consists of five key components:

1.  **Ingestion Engine ("The River Streams")**: Scrapes URLs and cleans HTML content from bookmarks.
2.  **Embedding Layer ("The Transformation")**: Converts raw text into numerical vectors using machine learning models.
3.  **Indexing ("The Depth of the Lake")**: Algorithms to organize vectors for fast nearest-neighbor search.
4.  **Storage Engine ("The Bedrock")**: Persists the data to disk so it survives a restart.
5.  **API ("The Shore")**: A command-line interface to query the database.

### Indexing
The core database aspect of Mizumi requires searching for the nearest neighbors efficiently

In this regard:
- HNSW: Hierarchically navigable small world is an algorithmic indexing approach that helps achieve this mission

---



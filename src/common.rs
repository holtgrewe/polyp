//! Functionality shared across the crate.

use std::str::FromStr;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use hpo::{
    similarity::{Builtins, StandardCombiner},
    term::InformationContentKind,
};
use strum::{EnumIter, IntoEnumIterator};

/// Shared command line arguments.
#[derive(Parser, Debug)]
pub struct Args {
    /// Verbosity of the program
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

/// Load HPO either from binary `$path_hpo/hpo.bin` if it exist, otherwise load as
/// standard directory from `$path_hpo`.
///
/// # Errors
///
/// In the case of loading failure.
pub fn load_hpo<P: AsRef<std::path::Path>>(path: P) -> Result<hpo::Ontology, anyhow::Error> {
    if path.as_ref().join("hpo.bin").exists() {
        tracing::info!(
            "  attempting to load binary HPO file from {}",
            path.as_ref().display()
        );
        Ok(hpo::Ontology::from_binary(path.as_ref().join("hpo.bin"))?)
    } else {
        tracing::info!(
            "  attempting to load HPO from standard file {}",
            path.as_ref().display()
        );
        Ok(hpo::Ontology::from_standard(&format!(
            "{}",
            path.as_ref().display()
        ))?)
    }
}

/// Enum for representing the information content kind.
///
/// We replicate what is in the `hpo` create so we can put them on the command line and use
/// them in HTTP queries more easily.
#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    EnumIter,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum IcBasedOn {
    /// Compute information content based on gene.
    #[default]
    #[display("gene")]
    Gene,
    /// Compute information content based on OMIM disease.
    #[display("omim")]
    Omim,
}

impl FromStr for IcBasedOn {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        IcBasedOn::iter()
            .find(|m| m.to_string().as_str().eq(s))
            .ok_or(anyhow::anyhow!("unknown information content base: {}", s))
    }
}

/// Enum for representing similarity method to use.
///
/// We replicate what is in the `hpo` create so we can put them on the command line and use
/// them in HTTP queries more easily.
#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    EnumIter,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum SimilarityMethod {
    /// "Distance" similarity.
    #[display("distance")]
    DistanceGene,
    /// Graph IC similarity.
    #[display("graph-ic")]
    GraphIc,
    /// Information coefficient similarity..
    #[display("information-coefficient")]
    InformationCoefficient,
    /// Jiang & Conrath similarity.
    #[display("jc")]
    Jc,
    /// Lin similarity..
    #[display("lin")]
    Lin,
    /// "Mutation" similarity.
    #[display("mutation")]
    Mutation,
    /// "Relevance" similarity.
    #[display("relevance")]
    Relevance,
    /// Resnik similarity..
    #[default]
    #[display("resnik")]
    Resnik,
}

/// Convert to pairwise similarity.
pub fn to_pairwise_sim(sim: SimilarityMethod, ic_based_on: IcBasedOn) -> Builtins {
    let kind = match ic_based_on {
        IcBasedOn::Gene => InformationContentKind::Gene,
        IcBasedOn::Omim => InformationContentKind::Omim,
    };
    match sim {
        SimilarityMethod::DistanceGene => Builtins::Distance(kind),
        SimilarityMethod::GraphIc => Builtins::GraphIc(kind),
        SimilarityMethod::InformationCoefficient => Builtins::InformationCoefficient(kind),
        SimilarityMethod::Jc => Builtins::Jc(kind),
        SimilarityMethod::Lin => Builtins::Lin(kind),
        SimilarityMethod::Mutation => Builtins::Mutation(kind),
        SimilarityMethod::Relevance => Builtins::Relevance(kind),
        SimilarityMethod::Resnik => Builtins::Resnik(kind),
    }
}

impl FromStr for SimilarityMethod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SimilarityMethod::iter()
            .find(|m| m.to_string().as_str().eq(s))
            .ok_or(anyhow::anyhow!("unknown similarity method: {}", s))
    }
}

/// Representation of the standard combiners from HPO.
///
/// We replicate what is in the `hpo` create so we can put them on the command line and use
/// them in HTTP queries more easily.
#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    EnumIter,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum ScoreCombiner {
    /// funSimAvg algborithm.
    #[default]
    #[display("fun-sim-avg")]
    FunSimAvg,
    /// funSimMax algorithm.
    #[display("fun-sim-max")]
    FunSimMax,
    /// BMA algorithm.
    #[display("bma")]
    Bma,
}

impl From<ScoreCombiner> for StandardCombiner {
    fn from(val: ScoreCombiner) -> Self {
        match val {
            ScoreCombiner::FunSimAvg => StandardCombiner::FunSimAvg,
            ScoreCombiner::FunSimMax => StandardCombiner::FunSimMax,
            ScoreCombiner::Bma => StandardCombiner::Bma,
        }
    }
}

impl FromStr for ScoreCombiner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ScoreCombiner::iter()
            .find(|m| m.to_string().as_str().eq(s))
            .ok_or(anyhow::anyhow!("unknown score combiner: {}", s))
    }
}

/// The version of `viguno` package.
#[cfg(not(test))]
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the current version of `viguno`.
///
/// This allows us to override the version to `0.0.0` in tests.
pub fn version() -> &'static str {
    #[cfg(test)]
    return "0.0.0";
    #[cfg(not(test))]
    return VERSION;
}

/// Version information that is returned by the HTTP server.
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Default, Debug, Clone)]
pub struct Version {
    /// Version of the HPO.
    pub hpo: String,
    /// Version of the `viguno` package.
    pub viguno: String,
}

impl Version {
    /// Construct a new version.
    ///
    /// The viguno version is filed automatically.
    pub fn new(hpo: &str) -> Self {
        Self {
            hpo: hpo.to_string(),
            viguno: version().to_string(),
        }
    }
}

/// Code related to the HGNC xlink table.
pub mod hgnc_xlink {
    use std::collections::HashMap;

    /// Data structure for representing an entry of the table.
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde_with::skip_serializing_none]
    pub struct Entry {
        /// HGNC gene ID.
        pub hgnc_id: String,
        /// Ensembl gene ID.
        pub ensembl_gene_id: Option<String>,
        /// Entrez gene ID.
        #[serde(alias = "entrez_id")]
        pub ncgi_gene_id: Option<u32>,
        /// Gene symbol.
        pub gene_symbol: String,
    }

    /// Read the `hgnc_xlink.tsv` file using the `csv` crate via serde.
    ///
    /// # Errors
    ///
    /// In the case that the file could not be read.
    pub fn load_entries<P: AsRef<std::path::Path>>(path: &P) -> Result<Vec<Entry>, anyhow::Error> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(true)
            .from_path(path.as_ref())?;
        let mut entries = Vec::new();
        for result in rdr.deserialize() {
            let entry: Entry = result?;
            entries.push(entry);
        }
        Ok(entries)
    }

    /// Read the `hgnc_xlink.tsv` into a map from NCBI gene ID to HGNC gene ID.
    ///
    /// # Errors
    ///
    /// In the case that the file could not be read.
    pub fn load_ncbi_to_hgnc<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<HashMap<u32, String>, anyhow::Error> {
        let mut map = HashMap::new();
        for entry in load_entries(&path)? {
            if let Some(ncbi_gene_id) = entry.ncgi_gene_id {
                map.insert(ncbi_gene_id, entry.hgnc_id);
            }
        }
        Ok(map)
    }

    /// Uility function to make the inverse of a `HashMap`.
    pub fn inverse_hashmap<K, V, S>(map: &HashMap<K, V, S>) -> HashMap<V, K, S>
    where
        K: std::hash::Hash + Eq + Clone,
        V: std::hash::Hash + Eq + Clone,
        S: std::hash::BuildHasher + Default,
    {
        map.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
    }
}

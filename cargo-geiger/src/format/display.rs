use crate::format::pattern::Pattern;
use crate::format::Chunk;
use crate::mapping::{
    CargoMetadataParameters, GetLicenceFromCargoMetadataPackageId,
    GetPackageNameFromCargoMetadataPackageId,
    GetPackageVersionFromCargoMetadataPackageId,
    GetRepositoryFromCargoMetadataPackageId,
};

use cargo_metadata::PackageId;
use std::fmt;

pub struct Display<'a> {
    pub cargo_metadata_parameters: &'a CargoMetadataParameters<'a>,
    pub pattern: &'a Pattern,
    pub package: &'a PackageId,
}

impl<'a> fmt::Display for Display<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for chunk in &self.pattern.0 {
            match *chunk {
                Chunk::License => {
                    if let Some(ref license) = self
                        .cargo_metadata_parameters
                        .krates
                        .get_licence_from_cargo_metadata_package_id(
                            self.package,
                        )
                    {
                        (write!(fmt, "{}", license))?
                    }
                }
                Chunk::Package => {
                    (write!(
                        fmt,
                        "{} {}",
                        self.cargo_metadata_parameters
                            .krates
                            .get_package_name_from_cargo_metadata_package_id(
                                self.package
                            )
                            .unwrap(),
                        self.cargo_metadata_parameters
                            .krates
                            .get_package_version_from_cargo_metadata_package_id(
                                self.package
                            )
                            .unwrap()
                    ))?
                }
                Chunk::Raw(ref s) => (fmt.write_str(s))?,
                Chunk::Repository => {
                    if let Some(ref repository) = self
                        .cargo_metadata_parameters
                        .krates
                        .get_repository_from_cargo_metadata_package_id(
                            self.package,
                        )
                    {
                        (write!(fmt, "{}", repository))?
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod display_tests {
    use super::*;

    use crate::format::pattern::Pattern;
    use crate::format::Chunk;

    use cargo_metadata::{CargoOpt, MetadataCommand};
    use krates::Builder as KratesBuilder;
    use rstest::*;

    #[rstest(
        input_pattern,
        expected_formatted_string,
        case(
            Pattern(vec![Chunk::License]),
            "Apache-2.0/MIT"
        ),
        case(
            Pattern(vec![Chunk::Package]),
            "cargo-geiger 0.10.2"
        ),
        case(
            Pattern(vec![Chunk::Raw(String::from("chunk_value"))]),
            "chunk_value"
        ),
        case(
            Pattern(vec![Chunk::Repository]),
            "https://github.com/rust-secure-code/cargo-geiger"
        )
    )]
    fn display_format_fmt_test(
        input_pattern: Pattern,
        expected_formatted_string: &str,
    ) {
        let metadata = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .features(CargoOpt::AllFeatures)
            .exec()
            .unwrap();

        let krates = KratesBuilder::new()
            .build_with_metadata(metadata.clone(), |_| ())
            .unwrap();

        let package_id = metadata.root_package().unwrap().id.clone();

        let display = Display {
            cargo_metadata_parameters: &CargoMetadataParameters {
                krates: &krates,
                metadata: &metadata,
            },
            pattern: &input_pattern,
            package: &package_id,
        };

        assert_eq!(format!("{}", display), expected_formatted_string);
    }
}

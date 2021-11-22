fn main() -> anyhow::Result<()> {
    // Before using any pylon methods, the pylon runtime must be initialized.
    let pylon = pylon_cxx::Pylon::new();

    // Create an instant camera object with the camera device found first.
    let camera = pylon_cxx::TlFactory::instance(&pylon).create_first_device()?;

    // Print the model name of the camera.
    println!("Using device {}.", camera.device_info().model_name()?);

    camera.open()?;

    let filename = "NodeMap.pfs";

    println!("Saving camera's node map to file \"{}\".", filename);
    pylon_cxx::NodeMap::save(&camera, filename)?;

    println!("Reading file back to camera's node map.");
    pylon_cxx::NodeMap::load(&camera, filename, true)?;

    println!("Ok.");
    Ok(())
}

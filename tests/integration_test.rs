use mix_system::mixal::assemble::assemble_file;

#[test]
fn test_mixal() -> anyhow::Result<()> {
    assemble_file("test_data/findmax.mixal")
}

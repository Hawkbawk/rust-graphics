use vulkano::instance::{ InstanceExtensions, Instance, PhysicalDevice };

fn main() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance!");
    for device in PhysicalDevice::enumerate(&instance) {
        println!("{:?}", device)
    }
    let physical = PhysicalDevice::enumerate(&instance).next().expect("Couldn't enumerate on devices");
}

use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

fn main() -> Result<(), Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(Error::I2c)?;

    let delay = Delay;

    let mut mpu = Mpu6050::new(i2c, delay);
    mpu.init()?;

    loop {
        // get roll and pitch estimate
        let acc = mpu.get_acc_angles()?;
        println!("r/p: {:?}", acc);

        // get temp
        let temp = mpu.get_temp()?;
        println!("temp: {}c", temp);

        // get gyro data, scaled with sensitivity 
        let gyro = mpu.get_gyro()?;
        println!("gyro: {:?}", gyro);
        
        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc()?;
        println!("acc: {:?}", acc);
    }
}
use capsules::lsm6dsoxtr::Lsm6dsoxtrI2C;
use capsules::virtual_i2c::I2CDevice;
use core::mem::MaybeUninit;
use kernel::capabilities;
use kernel::component::Component;
use kernel::{create_capability, static_init_half};

// Setup static space for the objects.
#[macro_export]
macro_rules! lsm6ds_i2c_component_helper {
    ($i2c_mux:expr, $accelerometer_address:expr $(,)?) => {{
        use capsules::lsm6dsoxtr::Lsm6dsoxtrI2C;
        use capsules::virtual_i2c::I2CDevice;
        use core::mem::MaybeUninit;

        static mut BUFFER: [u8; 8] = [0; 8];

        let accelerometer_i2c =
            components::i2c::I2CComponent::new($i2c_mux, $accelerometer_address)
                .finalize(components::i2c_component_helper!());
        
        static mut lsm6dsoxtr: MaybeUninit<Lsm6dsoxtrI2C<'static>> = MaybeUninit::uninit();
        (
            &accelerometer_i2c,
            &mut BUFFER,
            &mut lsm6dsoxtr,
        )
    }};

    ($i2c_mux:expr $(,)?) => {{
        $crate::lsm6ds_i2c_component_helper!(
            $i2c_mux,
            capsules::lsm6ds::ACCELEROMETER_BASE_ADDRESS
        )
    }};
}

pub struct Lsm6dsoxtrI2CComponent {
    board_kernel: &'static kernel::Kernel,
    driver_num: usize,
}

impl Lsm6dsoxtrI2CComponent {
    pub fn new(board_kernel: &'static kernel::Kernel, driver_num: usize) -> Lsm6dsoxtrI2CComponent {
        Lsm6dsoxtrI2CComponent {
            board_kernel: board_kernel,
            driver_num,
        }
    }
}

impl Component for Lsm6dsoxtrI2CComponent {
    type StaticInput = (
        &'static I2CDevice<'static>,
        &'static mut [u8],
        &'static mut MaybeUninit<Lsm6dsoxtrI2C<'static>>,
    );
    type Output = &'static Lsm6dsoxtrI2C<'static>;

    unsafe fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
        let grant = self.board_kernel.create_grant(self.driver_num, &grant_cap);
        let lsm6dsox = static_init_half!(
            static_buffer.2,
            Lsm6dsoxtrI2C<'static>,
            Lsm6dsoxtrI2C::new(static_buffer.0, static_buffer.1, grant)
        );
        static_buffer.0.set_client(lsm6dsox);

        lsm6dsox
    }
}

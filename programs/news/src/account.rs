use anchor_lang::prelude::*;
use crate::constants::*;

#[account(zero_copy)]
// #[repr(packed)]
pub struct Pool {
  pub news_id: u64, 
  pub reporter: Pubkey,
  pub created_at: u32,
  pub updated_at: u32,
  pub state: u32
}

impl Default for Pool {
  #[inline]
  fn default() -> Pool {
      Pool {
          news_id: 0,
          reporter: anchor_lang::solana_program::pubkey!("hFPVCyAan8HVuC3AVM34usCqUcEnqmDjdkHPk5NrAKW"),
          created_at: 0,
          updated_at: 0,
          state: 0
      }
  }
}

#[account(zero_copy)]
// #[repr(packed)]
pub struct OwnerVault {
  pub owner: Pubkey,
  pub balance: u64
}

impl Default for OwnerVault {
  #[inline]
  fn default() -> OwnerVault {
     OwnerVault {
          owner: anchor_lang::solana_program::pubkey!("3ttYrBAp5D2sTG2gaBjg8EtrZecqBQSBuFRhsqHWPYxX"),
          balance: 0
      }
  }
}

#[account(zero_copy)]
// #[repr(packed)]
pub struct Vault {
  pub reporter: Pubkey,
  pub balance: u64
}

impl Default for Vault {
  #[inline]
  fn default() -> Vault {
      Vault {
          reporter: anchor_lang::solana_program::pubkey!("1294RwniF1cjKmsPLU1dHHaYXTCvrCQt8uY9tfuaCMPi"),
          balance: 0
      }
  }
}

#[account(zero_copy)]
// #[repr(packed)]
pub struct User {
  pub reporters: [Reporter; MAX_REPORTER_COUNT],
  pub count: u32
}

impl Default for User {
  #[inline]
  fn default() -> User {
    User {
          reporters: [
            Reporter {
                    ..Default::default()
                }; MAX_REPORTER_COUNT
          ],
          count: 0
      }
  }
}

impl User {
  pub fn find_reporter(&self, reporter: Pubkey) -> usize {
    let mut index = MAX_REPORTER_COUNT;
    for i in 0..self.count as usize {
      if self.reporters[i].reporter == reporter {
        index = i;
        break;
      }
    }

    index
  }

  pub fn validate_reporter(&self, reporter: Pubkey, role: u32) -> Result<bool> {
    let mut result = true;
    // for i in 0..self.count as usize {
    //   if self.reporters[i].reporter.key() == reporter.key() && self.reporters[i].role == role {
    //     result = true;
    //     break;
    //   }
    // }

    msg!("validate result {}", result);
    msg!("reporter {}", reporter.key());
    msg!("role {}", role);

    Ok(result)
  }

  pub fn add_reporter(&mut self, reporter: Pubkey, role: u32) -> Result<bool> {
    let index = self.find_reporter(reporter);
    if index < MAX_REPORTER_COUNT {
      return Ok(false);
    }
    self.reporters[self.count as usize] = Reporter {
      reporter,
      role,
    };

    Ok(true)
  }

  pub fn edit_reporter(&mut self, old_reporter: Pubkey, reporter: Pubkey, role: u32) -> Result<bool> {
    let index = self.find_reporter(old_reporter);

    if index < MAX_REPORTER_COUNT {
      msg!("edit_reporter");
      msg!("role {}", role);
      msg!("reporter {}", reporter);
      self.reporters[index].reporter = reporter;
      self.reporters[index].role = role;
    } else {
      msg!("add_reporter");
      msg!("reporter {}", reporter);
      msg!("role {}", role);
      self.add_reporter(reporter, role);
    }

    Ok(true)
  }

  pub fn delete_reporter(&mut self, reporter: Pubkey) -> Result<bool> {
    let index = self.find_reporter(reporter);

    if index < MAX_REPORTER_COUNT {
      for i in index..self.count as usize - 1 {
        self.reporters[i] = self.reporters[i + 1];
      }
    }

    return Ok(true);
  }
}

#[account(zero_copy)]
// #[repr(packed)]
pub struct CampaignPool {
  pub campaign_id: u64, 
  pub advertiser: Pubkey,
  pub created_at: u32,
  pub updated_at: u32,
  pub state: u32,
}

impl Default for CampaignPool {
  #[inline]
  fn default() -> CampaignPool {
    CampaignPool {
          campaign_id: 0,
          advertiser: anchor_lang::solana_program::pubkey!("B9NhXx7Dq3JJ7kRw9dEXNU4BR3vdMxeqVWoJc97itfGY"),
          created_at: 0,
          updated_at: 0,
          state: 0,
      }
  }
}

#[zero_copy]
#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct Reporter {
  pub reporter: Pubkey,
  pub role: u32 //  1: senior, 2: admin
}
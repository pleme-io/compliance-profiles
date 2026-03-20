control 'sshd-02' do
  impact 0.7
  title 'SSH MaxAuthTries'
  desc 'Setting the MaxAuthTries parameter to a low number minimizes the risk
        of successful brute-force attacks to the SSH server. It is recommended
        to set MaxAuthTries to 4 or fewer.'
  tag nist: ['AC-7', 'CM-6']
  tag cis: ['5.2.7']

  only_if('SSH server must be installed') do
    package('openssh-server').installed? || command('sshd').exist?
  end

  describe sshd_config do
    its('MaxAuthTries') { should cmp <= 4 }
  end
end

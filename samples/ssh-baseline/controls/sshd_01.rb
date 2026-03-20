control 'sshd-01' do
  impact 1.0
  title 'SSH Protocol Version'
  desc 'Ensure SSH Protocol is set to 2. SSH Protocol version 1 suffers from
        design flaws that result in security vulnerabilities. Only SSH Protocol
        version 2 connections should be permitted.'
  tag nist: ['AC-17(2)', 'SC-8']
  tag cis: ['5.2.4']

  only_if('SSH server must be installed') do
    package('openssh-server').installed? || command('sshd').exist?
  end

  describe sshd_config do
    its('Protocol') { should cmp 2 }
  end
end

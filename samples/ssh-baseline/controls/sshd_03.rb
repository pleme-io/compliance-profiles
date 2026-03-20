control 'sshd-03' do
  impact 1.0
  title 'SSH Root Login'
  desc 'Disallowing root logins over SSH requires system admins to authenticate
        using their own individual account then escalating to root via sudo.
        This in turn limits opportunity for non-repudiation and provides a
        clear audit trail.'
  tag nist: ['AC-6(2)', 'IA-2']
  tag cis: ['5.2.10']

  only_if('SSH server must be installed') do
    package('openssh-server').installed? || command('sshd').exist?
  end

  describe sshd_config do
    its('PermitRootLogin') { should eq 'no' }
  end
end

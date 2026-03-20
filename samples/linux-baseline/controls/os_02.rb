control 'os-02' do
  impact 0.7
  title 'Check OS Release'
  desc 'The operating system must be a supported release to ensure security
        patches and updates are available from the vendor.'
  tag nist: ['CM-6', 'SI-2']
  tag cis: ['1.1']

  describe os.family do
    it { should eq 'linux' }
  end

  describe command('uname -m') do
    its('stdout') { should match(/x86_64|aarch64/) }
  end
end


clean:
	@find inputs -type f | xargs dos2unix
	@find test_data -type f | xargs dos2unix
	@find inputs -type f | xargs perl -pi -e 'chomp if eof'
	@find test_data -type f | xargs perl -pi -e 'chomp if eof'



package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var (
	cfgFile string

	rootCmd = &cobra.Command{
		Use:   "aoc-cli",
		Short: "aoc-cli is a AdventOfCode cli",
		Long: `A easy way to generate AdventOfCode
				Golang structure for each day`,
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println("AOC CLI v0.1 -- HEAD")
		},
	}
)

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}

func init() {
	cobra.OnInitialize(initConfig)

	rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.aoc.yaml)")
	rootCmd.PersistentFlags().Bool("viper", true, "use viper for configuration")
	err := viper.BindPFlag("viper", rootCmd.PersistentFlags().Lookup("viper"))
	cobra.CheckErr(err)
}

func initConfig() {
	if cfgFile != "" {
		viper.SetConfigFile(cfgFile)
	} else {
		home, err := os.UserHomeDir()
		cobra.CheckErr(err)

		// Search config in home directory with name ".aoc" (without extension)
		viper.AddConfigPath(home)
		viper.SetConfigFile("yaml")
		viper.SetConfigName(".aoc")
	}

	viper.AutomaticEnv()

	if err := viper.ReadInConfig(); err == nil {
		fmt.Println("Using config file:", viper.ConfigFileUsed())
	}
}
